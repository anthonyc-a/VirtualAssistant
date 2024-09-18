<script lang="ts">
  import { fly, fade, scale } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import {
    Paperclip,
    Send,
    User,
    Mail,
    MessageSquare,
    X,
    Briefcase,
    CheckCircle,
    Lightbulb,
    Folder,
  } from "lucide-svelte";

  export let projectTitle: string = "";
  export let message: string = "";
  export let messaging: boolean = false;
  export let name: string = "";
  export let email: string = "";

  let isFocused: boolean = false;
  let files: File[] = [];
  let fileError: string = "";
  let errors: string[] = [];
  let nameFocused = false;
  let emailFocused = false;
  let messageFocused = false;
  let showConfirmation = false;

  const MAX_FILE_SIZE = 5 * 1024 * 1024; // 5MB
  const MAX_FILES = 3;

  $: isProjectTitleValid = projectTitle.trim().length > 0;
  $: if (isFocused) {
    document.body.style.overflow = "hidden";
  } else {
    document.body.style.overflow = "";
  }

  function handleFocus() {
    isFocused = true;
  }

  function handleBlur() {
    if (!projectTitle && !message && !name && !email && files.length === 0) {
      isFocused = false;
    }
  }

  function validateForm() {
    errors = [];
    if (!projectTitle.trim()) errors.push("Project title is required");
    if (!name.trim()) errors.push("Name is required");
    if (!email.trim()) errors.push("Email is required");
    else if (!/\S+@\S+\.\S+/.test(email)) errors.push("Invalid email format");
    if (!message.trim()) errors.push("Message is required");
    return errors.length === 0;
  }

  function handleSubmit() {
    if (isProjectTitleValid && validateForm()) {
      console.log("Submitting:", { projectTitle, message, name, email, files });
      isFocused = false;
      setTimeout(() => {
        showConfirmation = true;
        setTimeout(() => {
          projectTitle = "";
          message = "";
          name = "";
          email = "";
          files = [];
          fileError = "";
          showConfirmation = false;
        }, 2000);
      }, 300); // Wait for collapse animation before showing confirmation
    }
  }

  function handleFileChange(event: Event) {
    if (!isProjectTitleValid) return;
    const target = event.target as HTMLInputElement;
    if (target.files) {
      const newFiles = Array.from(target.files);
      if (files.length + newFiles.length > MAX_FILES) {
        fileError = `You can only attach up to ${MAX_FILES} files`;
        return;
      }
      const invalidFiles = newFiles.filter((file) => file.size > MAX_FILE_SIZE);
      if (invalidFiles.length > 0) {
        fileError = "One or more files exceed 5MB limit";
      } else {
        files = [...files, ...newFiles];
        fileError = "";
      }
    }
    target.value = "";
  }

  function removeFile(index: number) {
    files = files.filter((_, i) => i !== index);
    fileError = "";
  }

  function notMessaging() {
    messaging = false;
  }
</script>

{#if messaging}
  <div
    role="presentation"
    on:click={notMessaging}
    class="fixed top-0 left-0 w-full h-screen bg-black/80"
  ></div>

  {#if isFocused}
    <div
      transition:fade={{ duration: 200 }}
      class="fixed inset-0 bg-black bg-opacity-75 z-[9999]"
      on:click={() => (isFocused = false)}
      role="presentation"
    ></div>
  {/if}

  {#if showConfirmation}
    <div
      in:scale={{ duration: 300, start: 0.5, opacity: 0, easing: cubicOut }}
      out:fade={{ duration: 300 }}
      class="fixed bottom-24 left-1/2 -translate-x-1/2 z-[10000] bg-green-500 text-white px-4 py-2 rounded-full flex items-center space-x-2"
    >
      <CheckCircle size={18} />
      <span>Message sent successfully!</span>
    </div>
  {/if}

  <form
    on:submit|preventDefault={handleSubmit}
    class="w-[95%] md:w-full overflow-scroll md:overflow-hidden fixed bottom-[88px] left-1/2 -translate-x-1/2 z-[9999] max-w-xl bg-accent bg-opacity-90 backdrop-blur text-left border border-border text-foreground rounded-full"
    style="height: {isFocused ? 'auto' : '3.4rem'}; max-height: {isFocused
      ? '35vh'
      : '3.5rem'}; border-radius: {isFocused
      ? '20px'
      : '56px'}; border-color: {fileError || errors.length > 0
      ? 'rgb(248 113 113)'
      : 'transparent' &&
        projectTitle &&
        'var(--foreground)'}; transition: max-height 0.4s ease;"
  >
    <div class="p-5 py-2.5 border-b border-muted flex items-center">
      <Folder class="text-muted-foreground mr-3" size={18} />
      <input
        placeholder="Describe your project here..."
        class="w-full bg-transparent !outline-none text-[16px] placeholder:tracking-[0.1px] placeholder:text-muted-foreground text-foreground"
        bind:value={projectTitle}
        on:focus={handleFocus}
        on:blur={handleBlur}
      />
      <button
        type="submit"
        class="ml-2 -me-3 text-background hover:text-subaccent transition-colors bg-foreground rounded-full p-2 {isProjectTitleValid
          ? ''
          : 'opacity-100 cursor-not-allowed'}"
        aria-label="Send message"
        disabled={!isProjectTitleValid}
      >
        <Send size={17} />
      </button>
    </div>

    {#if isFocused}
      <div
        transition:fly={{ y: 10, duration: 500, easing: cubicOut }}
        class="p-4 space-y-6"
      >
        <div class="relative {isProjectTitleValid ? '' : 'opacity-50'}">
          <User
            size={18}
            class="absolute left-3 top-1/2 -translate-y-1/2 text-[#999]"
          />
          <input
            id="name"
            class="w-full bg-muted rounded-2xl p-3 pl-10 border border-muted-foreground !outline-none text-foreground pt-6"
            bind:value={name}
            disabled={!isProjectTitleValid}
            on:focus={() => (nameFocused = true)}
            on:blur={() => (nameFocused = false)}
          />
          <label
            for="name"
            class="absolute left-10 text-[#999] transition-all duration-200 {name ||
            nameFocused
              ? 'text-xs top-2'
              : 'top-1/2 -translate-y-1/2'}"
          >
            Your Name
          </label>
        </div>

        <div class="relative {isProjectTitleValid ? '' : 'opacity-50'}">
          <Mail
            size={18}
            class="absolute left-3 top-1/2 -translate-y-1/2 text-[#999]"
          />
          <input
            id="email"
            type="email"
            class="w-full bg-muted border border-muted-foreground rounded-2xl p-3 pl-10 !outline-none text-white pt-6"
            bind:value={email}
            disabled={!isProjectTitleValid}
            on:focus={() => (emailFocused = true)}
            on:blur={() => (emailFocused = false)}
          />
          <label
            for="email"
            class="absolute left-10 text-[#999] transition-all duration-200 {email ||
            emailFocused
              ? 'text-xs top-2'
              : 'top-1/2 -translate-y-1/2'}"
          >
            Your Email Address
          </label>
        </div>

        <div class="relative {isProjectTitleValid ? '' : 'opacity-50'}">
          <MessageSquare size={18} class="absolute left-3 top-5 text-[#999]" />
          <textarea
            id="message"
            class="w-full bg-muted border border-muted-foreground rounded-2xl p-3 pl-10 !outline-none text-foreground resize-none h-24 pt-6"
            bind:value={message}
            disabled={!isProjectTitleValid}
            on:focus={() => (messageFocused = true)}
            on:blur={() => (messageFocused = false)}
          ></textarea>
          <label
            for="message"
            class="absolute left-10 text-[#999] transition-all duration-200 {message ||
            messageFocused
              ? 'text-xs top-2'
              : 'top-5'}"
          >
            Enter your message here
          </label>
        </div>

        <div
          class="flex items-center space-x-3 p-3 bg-[#333] rounded-2xl {isProjectTitleValid
            ? ''
            : 'opacity-50'}"
        >
          <Paperclip size={18} class="text-[#999]" />
          <input
            type="file"
            on:change={handleFileChange}
            class="w-full bg-transparent text-white file:mr-4 file:py-1 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-[#555] file:text-white hover:file:bg-[#666] disabled:opacity-50"
            multiple
            disabled={!isProjectTitleValid}
          />
        </div>
        {#if files.length > 0}
          <div class="flex flex-wrap gap-2 px-3">
            {#each files as file, i}
              <div
                class="flex items-center bg-[#444] rounded-full px-3 py-1 text-xs"
              >
                <span class="truncate max-w-[150px]">{file.name}</span>
                <button
                  on:click={() => removeFile(i)}
                  class="ml-2 text-[#999] hover:text-white"
                >
                  <X size={14} />
                </button>
              </div>
            {/each}
          </div>
        {/if}
        {#if fileError || errors.length > 0}
          <div class="flex items-center text-red-400 text-xs px-3">
            <span class="mr-2">⚠️</span>
            {fileError || errors.join(", ")}
          </div>
        {/if}
      </div>
    {/if}
  </form>
{/if}
