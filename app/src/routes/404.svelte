<script>
	export let status = 404;
	export let error = { message: "Page Not Found" };
</script>

<h1>{status}</h1>
<p>{error.message}</p>
