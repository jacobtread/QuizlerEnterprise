import { ENDPOINTS, axiosInstance } from "./api";

// Confirmation responses for OID
export type OIDConfirmResponse =
	| {
			type: "Success";
			default_username: string | null;
	  }
	| { type: "Existing" };

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
	token: string;
	provider: AuthProvider;
}

export type OIDProviders = Partial<Record<AuthProvider, OIDProvider>>;

export interface OIDProvidersResponse {
	providers: OIDProviders;
}

export interface OIDProvider {
	auth_url: string;
}

export type OIDAuthenticateResponse =
	| { type: "CreateAccount"; token: string; default_username: string | null }
	| ({ type: "ExistingLinked" } & TokenResponse);

export interface BasicRegisterRequest {
	username: string;
	email: string;
	password: string;
}

export interface BasicLoginRequest {
	email: string;
	password: string;
}

export async function registerBasic(
	body: BasicRegisterRequest,
	captchaToken: string
): Promise<TokenResponse> {
	const { data } = await axiosInstance.post(ENDPOINTS.auth.basic.register, body, {
		headers: {
			"x-captcha-token": captchaToken
		}
	});

	return data;
}

export async function loginBasic(
	body: BasicLoginRequest,
	captchaToken: string
): Promise<TokenResponse> {
	const res = await axiosInstance.post(ENDPOINTS.auth.basic.login, body, {
		headers: {
			"x-captcha-token": captchaToken
		}
	});

	return res.data;
}

/**
 * Requests a new authorization token using the provided
 * refresh token
 *
 * @param refreshToken the refresh token
 * @returns The new token data
 */
export async function refreshToken(refreshToken: string): Promise<TokenResponse> {
	const { data } = await axiosInstance.post(ENDPOINTS.auth.token.refresh, {
		refresh_token: refreshToken
	});

	return data;
}

/**
 * Request the available OpenID providers
 *
 * @returns The confirmation result
 */
export async function openIdProviders(): Promise<OIDProvidersResponse> {
	const res = await axiosInstance.get(ENDPOINTS.auth.oid.providers);

	return res.data;
}

/**
 * Request confirmation of a successful OpenID login
 *
 * @param data The OpenID token and provider
 * @returns The confirmation result
 */
export async function openIdAuthenticate(
	code: string,
	provider: AuthProvider
): Promise<OIDAuthenticateResponse> {
	const res = await axiosInstance.post(ENDPOINTS.auth.oid.authenticate, {
		code,
		provider
	});

	return res.data;
}

/**
 * Request to create an account using an OpenID auth token
 *
 * @param data The OpenID token and provider
 * @param username The username to give the account
 * @param password The password to set for the account
 * @returns
 */
export async function openIdCreate(
	data: OIDData,
	username: string,
	password: string
): Promise<TokenResponse> {
	const res = await axiosInstance.post(ENDPOINTS.auth.oid.create, {
		...data,
		username,
		password
	});

	return res.data;
}
