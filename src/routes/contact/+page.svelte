<script>
  import Heading from "../../components/Heading.svelte";

  let name = "";
  let email = "";
  let message = "";

  const handleSubmit = (event) => {
    event.preventDefault();
    // Handle form submission logic here
    console.log({ name, email, message });
  };
</script>

<div class="max-w-2xl w-full mx-auto">
  <Heading
    heading="Get in Touch"
    subheading="Embracing an iterative approach to design and development."
    showBreak={false}
  />

  <div class="max-w-2xl mx-auto mt-0 p-6 bg-background rounded-lg">
    <form class="p-6 border border-border rounded-3xl" on:submit={handleSubmit}>
      <div class="mb-4">
        <label class="block text-foreground text-sm font-bold mb-2" for="name"
          >Name</label
        >
        <input
          class="shadow appearance-none border rounded w-full py-2 px-3 text-foreground leading-tight focus:outline-none focus:shadow-outline"
          id="name"
          type="text"
          bind:value={name}
          placeholder="Your name"
          required
        />
      </div>
      <div class="mb-4">
        <label class="block text-foreground text-sm font-bold mb-2" for="email"
          >Email</label
        >
        <input
          class="shadow appearance-none border rounded w-full py-2 px-3 text-foreground leading-tight focus:outline-none focus:shadow-outline"
          id="email"
          type="email"
          bind:value={email}
          placeholder="Your email"
          required
        />
      </div>
      <div class="mb-4">
        <label
          class="block text-foreground text-sm font-bold mb-2"
          for="message">Message</label
        >
        <textarea
          class="shadow appearance-none border rounded w-full py-2 px-3 text-foreground leading-tight focus:outline-none focus:shadow-outline"
          id="message"
          bind:value={message}
          placeholder="Your message"
          rows="5"
          required
        ></textarea>
      </div>
      <div class="flex items-center justify-between">
        <button
          class="bg-accent w-full hover:bg-accent-dark text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
          type="submit"
        >
          Send
        </button>
      </div>
    </form>
  </div>
</div>
