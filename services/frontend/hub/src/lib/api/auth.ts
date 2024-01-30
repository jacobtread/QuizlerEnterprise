import { makeRequest } from "./api";


// Confirmation responses for OID
export type OIDConfirmResponse =
    {
        type: "Success",
        default_username: string | null,
    }
    | { type: "Existing", token: string }
    | { type: "Conflict" };



export interface TokenResponse {
    token: string;
    refresh_token: string;
    expiry: number;
}

// Different auth providers
export enum AuthProvider {
    Google = "Google",
    Microsoft = "Microsoft"
}

// OpenID provider and token 
export interface OIDData {
    token: string,
    provider: AuthProvider,
}


/**
 * Requests a new authorization token using the provided 
 * refresh token
 * 
 * @param refreshToken the refresh token 
 * @returns The new token data
 */
export function refreshToken(refreshToken: string): Promise<TokenResponse> {
    return makeRequest({
        method: "POST",
        url: "/auth/oid/confirm",
        body: { refresh_token: refreshToken }
    })
}

/**
 * Request confirmation of a successful OpenID login
 * 
 * @param token The token granted through OpenID
 * @param provider The auth provider the token is for
 * @returns The confirmation result
 */
export function openIdConfirm(data: OIDData): Promise<OIDConfirmResponse> {
    return makeRequest({
        method: "POST",
        url: "/auth/oid/confirm",
        body: data
    })
}

/**
 * Request to create an account using an OpenID auth token
 * 
 * @param token The token granted through OpenID
 * @param provider The auth provider the token is for
 * @param username The username to give the account
 * @param password The password to set for the account
 * @returns 
 */
export function openIdCreate(
    data: OIDData,
    username: string,
    password: string,
): Promise<TokenResponse> {
    return makeRequest({
        method: "POST",
        url: "/auth/oid/create",
        body: { ...data, username, password }
    })
}


/**
 * Request to login to an account using an OpenID auth token
 * 
 * @param token The token granted through OpenID
 * @param provider The auth provider the token is for
 * @param username The username to give the account
 * @param password The password to set for the account
 * @returns 
 */
export function openIdLogin(
    data: OIDData,
): Promise<TokenResponse> {
    return makeRequest({
        method: "POST",
        url: "/auth/oid/login",
        body: data
    })
}

