import { writable, type Writable } from "svelte/store";
import { getUser as getUserAPI, type User } from "$lib/api/user";
import type { RequestError } from "$lib/api/api";
import { refreshToken, type TokenResponse } from "$lib/api/auth";
export type Token = string | null;

export let tokenData: TokenData | null = null;
let tokenRefreshTask: number | null = null;

export interface TokenData {
    token: string;
    refresh_token: string;
    expiry: number;
}

// Local storage key for the token value
const REFRESH_TOKEN_STORAGE_KEY: string = "quizler_refresh_token";

export const user: Writable<User> = writable(null!);

export async function loadUser(): Promise<boolean> {
    if (tokenData === null) return false;

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
    tokenData = null;
    localStorage.removeItem(REFRESH_TOKEN_STORAGE_KEY);
}

/**
 * Sets the currently stored authentication token
 * to the provided value and saves it to local storage
 * 
 * @param value The token value
 */
export function setTokenData(value: TokenResponse) {
    // Reset the stored user when the token changes 
    user.set(null!);
    tokenData = value;

    // Time in seconds to refresh early by
    const EARLY_REFRESH_DELAY_SECONDS = 30;

    // Determine when to refresh the token (Refresh early)
    const timestamp: number = Date.now();
    const refreshDelay: number = (value.expiry - timestamp) - EARLY_REFRESH_DELAY_SECONDS;

    // Cancel pending token refreshes
    if (tokenRefreshTask !== null) {
        clearTimeout(tokenRefreshTask);
    }

    // Queue the next token refresh
    tokenRefreshTask = setTimeout(() => {
        // Can't refresh without a refresh token
        if (tokenData === null) return;
        doTokenRefresh(tokenData.refresh_token);
    }, refreshDelay);

    localStorage.setItem(REFRESH_TOKEN_STORAGE_KEY, value.refresh_token);
}

/**
 * Handles refreshing the token using the provided
 * refresh token
 * 
 * @param token The refresh token
 */
async function doTokenRefresh(token: string) {
    try {
        const response = await refreshToken(token);
        setTokenData(response);
    } catch (e) {
        clearAuthToken();
        console.error("Failed to refresh token", e);
    }
}

/**
 * Loads the authentication from local storage updating
 * the related state
 */
export function loadAuthToken() {
    const refreshToken: Token = localStorage.getItem(REFRESH_TOKEN_STORAGE_KEY);
    // Ignore the token if its not set
    if (refreshToken == null) {
        return;
    }

    console.debug("Loaded localStorage refresh token", refreshToken);

    doTokenRefresh(refreshToken);
}

// Load the auth token
loadAuthToken();