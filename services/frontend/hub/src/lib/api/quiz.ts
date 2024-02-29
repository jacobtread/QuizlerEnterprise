import { createQuery } from "@tanstack/svelte-query";
import { ENDPOINTS, ServerResponseError, axiosInstance } from "./api";

export interface Quiz {
	id: number;
	title: string;
	description: string;
	state: QuizState;
	visibility: QuizVisibility;
	cover_image: string | null;
	data: unknown;
	owner: number;
	create_at: string;
	updated_at: string;
}

export const enum QuizState {
	Draft = 0,
	Published = 1
}

export const enum QuizVisibility {
	Private = 0,
	Public = 1
}

export async function createQuiz(title: string): Promise<Quiz> {
	const { data } = await axiosInstance.post(ENDPOINTS.quiz.create, { title }, {});

	return data;
}

export function useQuiz(id: number) {
	return createQuery({
		queryKey: ["quiz", id],
		queryFn: async () => {
			const { data } = await axiosInstance.get(ENDPOINTS.quiz.specific(id).root, {});
			return data as Quiz;
		},
		retry(_failureCount, error) {
			if (error instanceof ServerResponseError && [403, 404].includes(error.status)) {
				return false;
			}

			return true;
		}
	});
}
