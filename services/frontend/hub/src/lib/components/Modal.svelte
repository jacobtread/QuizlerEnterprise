<div class="modal">
	<div class="modal__inner">
		<slot />
	</div>
</div>

<style lang="scss">
	.modal {
		position: absolute;
		z-index: 99;
		left: 0;
		top: 0;
		width: 100%;
		height: 100%;

		display: flex;
		flex-flow: row;
		justify-content: center;
		align-items: center;
		background-color: rgba(0, 0, 0, 0.7);

		animation: modal 0.2s ease;

		&__inner {
			border-radius: 0.25rem;
			display: flex;
			flex-flow: column;
			gap: 0.5rem;
			animation: modalInner 0.2s ease;
			max-width: calc(400px + 3rem);
			padding: 1.5rem;
			background-color: #333;
		}
	}

	@keyframes modal {
		0% {
			opacity: 0;
		}
		100% {
			opacity: 1;
		}
	}

	@keyframes modalInner {
		0% {
			opacity: 0;
			transform: translateY(-14px);
		}

		50% {
			opacity: 0.3;
			transform: translateY(-7px);
		}

		100% {
			opacity: 1;
			transform: translateY(0px);
		}
	}
</style>
