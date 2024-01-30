
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