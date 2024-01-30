import { get, writable, type Writable } from "svelte/store";
import { getUser as getUserAPI, TokenReason, type User } from "$lib/api/user";
import type { RequestError } from "$lib/api/api";
import { goto } from "$app/navigation";
export type Token = string | null;

// Store for storing the current token value
export const tokenStore: Writable<Token> = writable(null);

// Store for storing the token loading state
export const tokenLoading: Writable<boolean> = writable(false);

// Local storage key for the token value
const TOKEN_STORAGE_KEY: string = "token";

export const user: Writable<User> = writable(null!);

export async function loadUser(): Promise<boolean> {
    const token: Token = get(tokenStore);
    if (!token) return false;

    let value: User;
    try {
        value = await getUserAPI();
    } catch (e) {
        const err = e as RequestError;
        switch (err[0]) {
            case 500:
                console.error("Server error occurred while attempting to authenticate", err[1]);
                break;
            case 401:
                console.error("Stored token is no longer valid", err[1]);
                clearAuthToken();
                break;
        }



        return false;
    }
    user.set(value);
    return true;
}


export function clearAuthToken() {
    user.set(null!); /* Reset the stored user when the token changes */
    tokenStore.set(null);
    localStorage.removeItem(TOKEN_STORAGE_KEY);
}

/**
 * Sets the currently stored authentication token
 * to the provided value and saves it to local storage
 * 
 * @param value The token value
 */
export function setAuthToken(value: string) {
    user.set(null!); /* Reset the stored user when the token changes */
    tokenStore.set(value);
    localStorage.setItem(TOKEN_STORAGE_KEY, value);
}

/**
 * Loads the authentication from local storage updating
 * the related state
 */
export function loadAuthToken() {
    const token: Token = localStorage.getItem(TOKEN_STORAGE_KEY);
    // Ignore the token if its not set
    if (token == null) {
        return;
    }

    console.debug("Loaded localStorage token", token);
    // Set the token state
    tokenStore.set(token)
    user.set(null!); /* Reset the stored user when the token changes */
}

// Load the auth token
loadAuthToken();