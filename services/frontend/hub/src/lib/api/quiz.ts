import { useQuery } from "@sveltestack/svelte-query";
import { ENDPOINTS, axiosInstance } from "./api";

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
	return useQuery(["quiz", id], async () => {
		const { data } = await axiosInstance.get(ENDPOINTS.quiz.specific(id).root, {});
		return data as Quiz;
	});
}
