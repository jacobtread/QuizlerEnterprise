import { ENDPOINTS, axiosInstance } from "./api";

/// Structure of users
export interface User {
	id: number;
	username: string;
	email: string;
	email_verified_at: number | null;
	role: string;
	created_at: string;
	updated_at: string;
}

export async function getSelfUser(): Promise<User> {
	const { data } = await axiosInstance.get(ENDPOINTS.user.self);

	return data;
}
