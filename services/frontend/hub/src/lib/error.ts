import { base } from "$app/paths";
import { goto } from "$app/navigation";

/**
 * Attempts to determine an error message from the 
 * provided error, defaults to "Unknown error"
 * 
 * @param err The error object
 * @returns The error message
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function getErrorMessage(err: any): string {
    if (err instanceof Error) {
        return err.message;
    } else {
        return "Unknown error";
    }
}

/**
 * Navigates to an error page with the provided error
 * details
 * 
 * @param name The error name 
 * @param description The error description
 * @param backURL URL for the back button
 * @returns Promise for the navigation
 */
export function gotoError(name?: string | undefined, description?: string | undefined, backURL?: string | undefined): Promise<void> {
    const params = new URLSearchParams();
    if (name !== undefined) {
        params.append("name", name);
    }
    if (description !== undefined) {
        params.append("description", description);
    }

    if (backURL !== undefined) {
        params.append("back", backURL);
    }

    const query = params.toString();
    return goto(`${base}/error?${query}`);
}