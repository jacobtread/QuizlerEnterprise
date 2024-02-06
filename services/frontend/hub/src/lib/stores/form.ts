import { ValidationError } from "$lib/api/api";
import { writable, type Writable } from "svelte/store";
import { ZodError, type ZodIssue } from "zod";

export type FormErrors = Partial<Record<string, string>>;

interface FormState {
    // Store for form errors
    errors: Writable<FormErrors>,

    // Store for when the form is loading
    loading: Writable<boolean>

    // Function to reset the form state
    reset: () => void;

    // Function to submit the form
    submit: () => Promise<void>,
}

type SubmitAction = () => Promise<void>;


export function createForm(submitAction: SubmitAction): FormState {
    const errors = writable({});
    const loading = writable(false);

    const submit = async (): Promise<void> => {
        // Update current state
        errors.set({});
        loading.set(true);

        try {
            // Submit the action
            await submitAction();
        } catch (e) {
            if (e instanceof ValidationError) {
                const data = e.data;

                // Merge with existing errors
                errors.update((existing: FormErrors) => ({ ...existing, ...data }));
            } else if (e instanceof ZodError) {
                const data: FormErrors = {};
                e.issues.forEach((issue: ZodIssue) => {
                    let pathOut = "";
                    let path: string | number = 0;
                    for (let i = 0; i < issue.path.length; i++) {
                        path = issue.path[i];
                        if (typeof path === "string") {
                            pathOut += path + ".";
                        } else {
                            pathOut += `[${path}]`;
                        }
                    }

                    if (typeof path === "string") {
                        pathOut = pathOut.substring(0, pathOut.length - 1);
                    }

                    data[pathOut] = issue.message;
                });


                // Merge with existing errors
                errors.update((existing: FormErrors) => ({ ...existing, ...data }));

            } else if (e instanceof Error) {
                const message = e.message;
                errors.update((errors: FormErrors) => {
                    errors["base"] = message;
                    return errors;
                })
            } else {
                console.error("Unknown error in form submission", e);
            }
        } finally {
            loading.set(false);
        }
    }


    const reset = () => {
        errors.set({});
        loading.set(false);
    }

    return {
        errors,
        loading,
        reset,
        submit,
    }

}
