import { tokenData } from "$lib/stores/auth";
import { PUBLIC_API_BASE_URL } from "$env/static/public"

// Http request method types
export type HttpMethod = "GET" | "POST" | "PUT" | "DELETE";

// Structure of errors from make request [statusCode, text]
export type RequestError = [number, string];

// Structure for a configuration object to provide to the
// makeRequest function
interface RequestConfig {
    // The request HTTP method to use
    method: HttpMethod;
    // The route segment of the URL
    url: string;
    // Optional body to encode as JSON
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    body?: any,
    /// Optional additional headers
    headers?: Record<string, string>,
}


// Error for server response errors
export class ServerResponseError extends Error {
    // The response status code
    status: number;

    constructor(status: number, message?: string | undefined, options?: ErrorOptions | undefined) {
        super(message, options);
        this.status = status;
    }
}

export class GenericError extends ServerResponseError {
    // The error name
    name: string;

    constructor(status: number, name: string, message: string, options?: ErrorOptions | undefined) {
        super(status, message, options);
        this.name = name;
    }
}

export class ValidationError extends GenericError {
    data: ValidationErrorData;

    constructor(status: number, name: string, message: string, data: ValidationErrorData, options?: ErrorOptions | undefined) {
        super(status, name, message, options);
        this.data = data;
    }

    getErrorMessage(path: ErrorPathSegment[]): string | null {
        const entry = this.data.errors
            .find((entry) => isMatchingSegments(entry[0], path));
        if (entry === undefined) return null
        return entry[1].message;
    }
}

/**
 * Checks that all the provided segments match
 * 
 * @param a The first list of segments
 * @param b The second list of segments
 * @returns Whether they match
 */
function isMatchingSegments(a: ErrorPathSegment[], b: ErrorPathSegment[]): boolean {
    if (a.length !== b.length) return false;

    for (let i = 0; i < a.length; i++) {
        if (!isSegmentEqual(a[i], b[i])) {
            return false;
        }

    }

    return true;
}

/**
 * Checks that both error paths segments match
 * 
 * @param a The first path segment
 * @param b The second path segment
 * @returns Whether they match
 */
function isSegmentEqual(a: ErrorPathSegment, b: ErrorPathSegment): boolean {
    return a[0] === b[0] && a[1] === b[1]
}


type ErrorPathSegment = [ErrorPathKind, string];
type ErrorPathKind = "none" | "key" | "index";
type ValidationErrorEntry = [ErrorPathSegment[], { message: string }]

interface ValidationErrorData {
    errors: ValidationErrorEntry[]
}


interface HttpErrorResponse<T> {
    name: string;
    message: string;
    data: T
}



/**
 * Makes a request with the provided details
 * 
 * @param method The HTTP method to use for the request
 * @param baseURL THe base portion of the URL
 * @param url The route portion of the URL
 * @param token The optional token to use for authentication
 * @param body The optional body to use 
 * @returns A promise for the provided type or an error
 */
export async function makeRequest<T>(config: RequestConfig): Promise<T> {
    const init: RequestInit = { method: config.method };
    const headers: Record<string, string> = config.headers ?? {};


    // Apply the token if provided
    if (tokenData !== null) {
        headers["Authorization"] = "Bearer " + tokenData.token;
    }

    // Serialize JSON body if provided
    if (config.method != "GET" && config.body) {
        headers["Content-Type"] = "application/json";
        init.body = JSON.stringify(config.body);
    }

    init.headers = headers;

    const url = new URL(config.url, PUBLIC_API_BASE_URL);

    let response: Response;
    // Handle initial fetch errors
    try {
        response = await fetch(url, init);
    } catch (e) {
        throw new Error("Failed to connect", { cause: e });
    }

    /// Handle 2xx status codes 
    if (response.ok) {
        // Handle invalid JSON responses
        try {
            return await response.json();
        } catch (e) {
            console.error("Invalid JSON response", e);
            throw new ServerResponseError(response.status, "Invalid server response", { cause: e });
        }
    }

    // Handle error response
    let responseText: string;
    try {
        responseText = await response.text();
    } catch (e) {
        console.error("Failed to get response text", e);
        throw new ServerResponseError(response.status, "Unknown error", { cause: e });
    }

    let responseJson: HttpErrorResponse<unknown>;
    try {
        responseJson = JSON.parse(responseText);
    } catch (e) {
        if (e instanceof SyntaxError) {
            // Handle non-JSON response types
            throw new ServerResponseError(response.status, responseText);
        } else {
            throw new ServerResponseError(response.status, "Unknown error", { cause: e });
        }
    }

    if (responseJson.name === "validation") {
        throw new ValidationError(response.status, responseJson.name, responseJson.message, (responseJson as HttpErrorResponse<ValidationErrorData>).data);
    } else {
        throw new GenericError(response.status, responseJson.name, responseJson.message)
    }

}