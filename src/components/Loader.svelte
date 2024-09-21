<script>
	import { BarLoader } from 'svelte-loading-spinners';
</script>

<div class="loader w-full max-w-[100vw] h-[100dvh] fixed top-0 left-0 bg-background z-[99999]">
	<div class="absolute hidden z-50 top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2">
		<BarLoader size="32"  unit="px" />
	</div>
</div>

<style>
	.loader {
		animation: disappear 0.3s 0.5s ease-in-out forwards;
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