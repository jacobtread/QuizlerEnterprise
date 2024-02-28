const paths = {
	auth: {
		login: "/auth/login",
		register: "/auth/register",
		openid: {
			complete: "/auth/openid/complete",
			finish: "/auth/openid/finish"
		}
	},
	root: "/",
	error: "/error",
	browse: "/browse",
	library: "/library",
	reports: "/reports",
	create: {
		root: "/create",
		specific: (id: string) => `/create/${id}`
	}
};

export default paths;
