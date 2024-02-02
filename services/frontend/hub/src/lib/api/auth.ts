import { makeRequest } from "./api";


// Confirmation responses for OID
export type OIDConfirmResponse =
    {
        type: "Success",
        default_username: string | null,
    } | { type: "Existing" };



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

export type OIDProviders = Partial<Record<AuthProvider, OIDProvider>>;

export interface OIDProvidersResponse {
    providers: OIDProviders;
}

export interface OIDProvider {
    auth_url: string;
}

export type OIDAuthenticateResponse =
    { type: "CreateAccount", token: string, default_username: string | null }
    | { type: "ExistingLinked" } & TokenResponse

export interface BasicRegisterRequest {
    username: string;
    email: string;
    password: string;
}

export interface BasicLoginRequest {
    email: string;
    password: string;
}


export function registerBasic(body: BasicRegisterRequest): Promise<TokenResponse> {
    return makeRequest({
        method: "POST",
        url: "/auth/basic/register",
        body: body
    })
}


export function loginBasic(body: BasicLoginRequest): Promise<TokenResponse> {
    return makeRequest({
        method: "POST",
        url: "/auth/basic/login",
        body: body
    })
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
        url: "/auth/token/refresh",
        body: { refresh_token: refreshToken }
    })
}

/**
 * Request the available OpenID providers
 * 
 * @returns The confirmation result
 */
export function openIdProviders(): Promise<OIDProvidersResponse> {
    return makeRequest({
        method: "GET",
        url: "/auth/oid/providers",
    })
}

/**
 * Request confirmation of a successful OpenID login
 * 
 * @param data The OpenID token and provider
 * @returns The confirmation result
 */
export function openIdAuthenticate(code: string, provider: AuthProvider): Promise<OIDAuthenticateResponse> {
    return makeRequest({
        method: "POST",
        url: "/auth/oid/authenticate",
        body: {
            code,
            provider
        }
    })
}

/**
 * Request to create an account using an OpenID auth token
 * 
 * @param data The OpenID token and provider
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
