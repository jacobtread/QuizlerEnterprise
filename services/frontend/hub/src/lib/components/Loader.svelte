<div class="loader">
	<div class="loader__spinner" />
</div>

<style lang="scss">
	.loader {
		position: absolute;
		left: 0;
		top: 0;
		width: 100%;
		height: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 99;
		background-color: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);

		&__spinner {
			display: block;
			width: 128px;
			height: 128px;
			border: 5px solid white;
			border-radius: 80px;
			border-top-color: transparent;

			animation: loader 1s infinite linear;
			position: relative;
		}
	}

	@keyframes loader {
		0% {
			transform: rotate(0);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
