import { ValidationError } from "$lib/api/api";
import { writable, type Writable } from "svelte/store";

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
