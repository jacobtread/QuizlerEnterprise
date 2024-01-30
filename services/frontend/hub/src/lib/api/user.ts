import { makeRequest } from "./api";

export const enum UserType {
    Default = 0x0,
    Google = 0x1
}

export const enum UserRole {
    Default = 0x0,
    Helper = 0x1,
    Admin = 0x2,
    SuperAdmin = 0x3,
}

export const USER_ROLE_NAMES: Record<UserRole, string> = {
    [UserRole.Default]: "Default",
    [UserRole.Helper]: "Helper",
    [UserRole.Admin]: "Admin",
    [UserRole.SuperAdmin]: "Super Admin",
}

export type PermissionType = "Default" | "Owned" | "Helper" | "Admin" | "SuperAdmin";

export interface Permission {
    type: PermissionType;
    expires?: string | null;
}

/// Structure of users
export interface User {
    id: number;
    username: string;
    email: string;
    email_verified_at: number | null;
    user_type: UserType;
    role: UserRole;
    created_at: string;
}

export function getUser(): Promise<User> {
    return makeRequest({
        method: "GET",
        url: "api/user",
    })
}

export interface UsernameResponse {
    exists: boolean;
}

export const enum TokenReason {
    Invalid = "Invalid token",
    Expired = "Expired token"
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