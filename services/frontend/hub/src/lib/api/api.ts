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
class ServerResponseError extends Error {
    // The response status code
    status: number;

    constructor(status: number, message?: string | undefined, options?: ErrorOptions | undefined) {
        super(message, options);
        this.status = status;
    }
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

    const token = tokenData;

    // Apply the token if provided
    if (token) {
        headers["Authorization"] = "Bearer " + token.token;
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

    // Handle non 200 status codes by taking the text response
    let text: string;
    try {
        text = await response.text();
    } catch (e) {
        console.error("Failed to get error response text", e);
        throw new ServerResponseError(response.status, "Unknown error", { cause: e });
    }

    throw new ServerResponseError(response.status, text);
}