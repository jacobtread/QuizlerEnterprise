<script lang="ts">
	import { base } from "$app/paths";
	import UserIcon from "~icons/solar/user-circle-bold-duotone";
	import { Popover, Avatar, Separator } from "bits-ui";

	import SettingsIcon from "~icons/solar/settings-bold-duotone";
	import LogoutIcon from "~icons/solar/logout-3-bold-duotone";

	import { clearAuthToken } from "$lib/stores/auth";

	import { user } from "$lib/stores/auth";

	function logout() {
		clearAuthToken();
	}
</script>

<header class="header">
	<div class="header__left">
		<!-- <Logo width="auto" height="2rem" /> -->
	</div>

	<div class="header__right">
		<nav class="nav">
			<a class="button" href="{base}/create">Create</a>

			<Popover.Root>
				<Popover.Trigger class="button">
					<Avatar.Root>
						<Avatar.Image />
						<Avatar.Fallback>
							<UserIcon width="30" height="30" color="#666" />
						</Avatar.Fallback>
					</Avatar.Root>
				</Popover.Trigger>
				<Popover.Content sideOffset={8}>
					<div class="profile">
						<Avatar.Root>
							<Avatar.Image />
							<Avatar.Fallback>
								<UserIcon width="30" height="30" color="#666" />
							</Avatar.Fallback>
						</Avatar.Root>
						<div>
							<p class="profile__name">{$user.username}</p>
							<p class="profile__email">{$user.email}</p>
						</div>
					</div>
					<Separator.Root />

					<a class="button" href="/settings">
						<SettingsIcon /> Settings
					</a>

					<Separator.Root />

					<a class="button" href="{base}/auth/login" on:click={logout}>
						<LogoutIcon /> Logout
					</a>
				</Popover.Content>
			</Popover.Root>
		</nav>
	</div>
</header>

<style lang="scss">
	@use "../assets/theme.scss" as theme;

	.profile {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem;
	}

	.profile__name {
		font-weight: bold;
		margin-bottom: 0.1rem;
	}

	.profile__email {
		color: #999;
		font-size: 0.8rem;
	}

	.header {
		background: #fff;

		border: 1px solid #ccc;
		padding: 0.5rem 1rem;

		display: flex;
		justify-content: space-between;

		z-index: 1;
	}

	.nav {
		display: flex;
	}

	.button {
		padding: 0.75rem;
		background-color: #fff;
		border-radius: 0.2rem;
		color: #444;
		text-decoration: none;

		display: flex;
		align-items: center;

		font-weight: bold;

		&.active {
			background-color: theme.$primaryColor;
			color: #ffffff;
		}

		&:hover {
			text-decoration: underline;
		}

		:global(svg) {
			margin-right: 0.5rem;
		}
	}
</style>
