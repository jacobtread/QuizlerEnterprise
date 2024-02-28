import { PUBLIC_API_BASE_URL } from "$env/static/public";
import axios, { AxiosError, type AxiosInstance } from "axios";

export const axiosInstance: AxiosInstance = axios.create({
	baseURL: PUBLIC_API_BASE_URL
});

export const ENDPOINTS = {
	auth: {
		basic: {
			register: "/auth/basic/register",
			login: "/auth/basic/login"
		},
		token: {
			refresh: "/auth/token/refresh"
		},
		oid: {
			providers: "/auth/oid/providers",
			authenticate: "/auth/oid/authenticate",
			create: "/auth/oid/create"
		}
	},
	user: {
		self: "/user/self"
	},
	quiz: {
		create: "/quiz/create",
		specific: (id: number) => ({
			root: `/quiz/${id}`
		})
	}
};

axiosInstance.interceptors.request.use(
	function (config) {
		return config;
	},
	function (error) {
		if (error instanceof Error) {
			return Promise.reject(error);
		} else {
			return Promise.reject(
				new Error(typeof error === "string" ? error : "Unknown error", { cause: error })
			);
		}
	}
);

// Transform error error responses
axiosInstance.interceptors.response.use(
	function (response) {
		return response;
	},
	function (err) {
		if (err instanceof AxiosError) {
			if (err.response) {
				const { data, status } = err.response;

				if (typeof data === "string") {
					return Promise.reject(new ServerResponseError(status, data, { cause: err }));
				}

				if (typeof data === "object") {
					const responseJson: HttpErrorResponse<unknown> = data as HttpErrorResponse<unknown>;

					if (responseJson.name === "validation") {
						return Promise.reject(
							new ValidationError(
								status,
								responseJson.name,
								responseJson.message,
								(responseJson as HttpErrorResponse<ValidationErrorData>).data
							)
						);
					} else {
						return Promise.reject(
							new GenericError(status, responseJson.name, responseJson.message)
						);
					}
				}
			}

			return Promise.reject(err);
		} else {
			console.error(err);
			return Promise.reject(new Error("Unknown error", { cause: err }));
		}
	}
);

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

	constructor(
		status: number,
		name: string,
		message: string,
		data: ValidationErrorData,
		options?: ErrorOptions | undefined
	) {
		super(status, name, message, options);
		this.data = data;
	}
}

type ValidationErrorData = Partial<Record<string, string>>;

interface HttpErrorResponse<T> {
	name: string;
	message: string;
	data: T;
}
