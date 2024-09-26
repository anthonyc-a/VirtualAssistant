<script>
	import { Moon } from 'svelte-loading-spinners';
</script>

<div class="loader w-full max-w-[100vw] h-[100dvh] fixed top-0 left-0 bg-background z-[99999]">
	<div class="absolute  invert dark:invert-0  z-50 top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2">
		<Moon size={28} unit="px" color="#fff"/>
	</div>
</div>

<style>
	.loader {
		animation: disappear 0.65s 1.25s ease-out forwards;
	}

	@keyframes disappear {
		0% {
			opacity: 1;
		}
		100% {
			opacity: 0;
			visibility: hidden;
		}
	}
</style>