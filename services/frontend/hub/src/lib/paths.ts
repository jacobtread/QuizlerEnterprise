import { base } from "$app/paths";

const paths = {
	auth: {
		login: `${base}/auth/login`,
		register: `${base}/auth/register`,
		openid: {
			complete: `${base}/auth/openid/complete`,
			finish: `${base}/auth/openid/finish`
		}
	},
	root: `${base}/`,
	error: `${base}/error`,
	browse: `${base}/browse`,
	library: `${base}/library`,
	reports: `${base}/reports`,
	create: {
		root: `${base}/create`,
		specific: (id: string) => `${base}/create/${id}`
	}
};

export default paths;
