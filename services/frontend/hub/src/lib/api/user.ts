import { makeRequest } from "./api";


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

export function getSelfUser(): Promise<User> {
    return makeRequest({
        method: "GET",
        url: "/user/self",
    })
}

export interface UsernameResponse {
    exists: boolean;
}

export function isUsernameTaken(username: string): Promise<UsernameResponse> {
    return makeRequest({
        method: "GET",
        url: "api/user/username",
        body: {
            username
        }
    })
}

export function issuePasswordReset(email: string) {
    return makeRequest({
        method: "POST",
        url: "api/user/password/reset",
        body: {
            email
        }
    })
}

export function changePassword(token: string, password: string) {
    return makeRequest({
        method: "POST",
        url: "api/user/password/change",
        body: {
            token,
            password
        }
    })
}