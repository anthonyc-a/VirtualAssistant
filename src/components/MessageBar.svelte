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
    Lightbulb,
    CheckCircle,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import Popup from "./Popup.svelte";

  export let projectTitle: string = "";
  export let message: string = "";
  export let messaging: boolean = false;
  export let name: string = "";
  export let email: string = "";

  let isFocused: boolean = false;
  let files: FileList | null = null;
  let fileError: string = "";
  let errors: string[] = [];
  let nameFocused = false;
  let emailFocused = false;
  let messageFocused = false;
  let showConfirmation = false;
  let isLoading = false;
  let isVisible = true;
  let lastScrollY = 0;

  onMount(() => {
    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      isVisible = currentScrollY < lastScrollY || currentScrollY < 50;
      lastScrollY = currentScrollY;
      console.log(isVisible);
    };

    window.addEventListener("scroll", handleScroll);

    return () => {
      window.removeEventListener("scroll", handleScroll);
    };
  });

  const MAX_FILE_SIZE = 5 * 1024 * 1024; // 5MB
  const MAX_FILES = 3;

  $: isProjectTitleValid = projectTitle.trim().length > 0;
  $: if (messaging) {
    document.body.style.overflow = "hidden";
  } else {
    document.body.style.overflow = "";
  }

  function handleFocus() {
    isFocused = true;
  }

  function handleBlur() {
    if (!projectTitle && !message && !name && !email && !files) {
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

  function handleFileChange(event: Event) {
    if (!isProjectTitleValid) return;
    const target = event.target as HTMLInputElement;
    if (target.files) {
      if (target.files.length > MAX_FILES) {
        fileError = `You can only attach up to ${MAX_FILES} files`;
        files = null;
      } else {
        const invalidFiles = Array.from(target.files).filter(
          (file) => file.size > MAX_FILE_SIZE
        );
        if (invalidFiles.length > 0) {
          fileError = "One or more files exceed 5MB limit";
          files = null;
        } else {
          files = target.files;
          fileError = "";
        }
      }
    }
  }

  async function handleSubmit() {
    if (isProjectTitleValid && validateForm()) {
      isLoading = true;
      errors = [];
      const formData = new FormData();
      formData.append("projectTitle", projectTitle);
      formData.append("name", name);
      formData.append("email", email);
      formData.append("message", message);
      if (files) {
        Array.from(files)
          .slice(0, MAX_FILES)
          .forEach((file) => {
            formData.append("files", file);
          });
      }

      try {
        const response = await fetch("/api/send-email", {
          method: "POST",
          body: formData,
        });

        const result = await response.json();

        if (result.success) {
          console.log("Email sent successfully");
          isFocused = false;
          showConfirmation = true;
          setTimeout(() => {
            projectTitle = "";
            message = "";
            name = "";
            email = "";
            files = null;
            fileError = "";
            showConfirmation = false;
          }, 2000);
        } else {
          console.error("Failed to send email:", result.message);
          errors = [result.message || "Failed to send email"];
        }
      } catch (error) {
        console.error("Error sending email:", error);
        errors = ["An error occurred. Please try again."];
      } finally {
        isLoading = false;
      }
    }
  }

  function removeFile(index: number) {
    if (files) {
      const dt = new DataTransfer();
      Array.from(files).forEach((file, i) => {
        if (i !== index) dt.items.add(file);
      });
      files = dt.files;
    }
    fileError = "";
  }

  function notMessaging() {
    messaging = false;
  }
</script>

{#if messaging}
  <div
    role="presentation"
    class="fixed top-0 z-[9999999] left-0 w-full h-screen bg-transparent backdrop-blur"
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
    <Popup {isVisible} />
  {/if}

  <form
    on:submit|preventDefault={handleSubmit}
    class="w-[calc(100%-40px)] z-[99999999] px-2 mt-6 fade-up mb-2.5 md:w-full !transition-all duration-300 overflow-scroll md:overflow-hidden fixed left-1/2 -translate-x-1/2 max-w-xl bg-accent bg-opacity-90 backdrop-blur text-left border border-border text-foreground rounded-full"
    style="height: {isFocused ? 'auto' : '3.4rem'}; max-height: {isFocused
      ? '55vh'
      : '3.5rem'}; border-radius: {isFocused
      ? '20px'
      : '56px'}; border-color: {fileError || errors.length > 0
      ? 'rgb(248 113 113)'
      : 'transparent' &&
        projectTitle &&
        'var(--foreground)'}; transition: max-height 0.4s ease;"
  >
    <div
      class="p-3 pr-4 py-2.5 border-b border-muted dark:border-border flex items-center"
    >
      <Lightbulb class="text-muted-foreground mr-3" size={20} />
      <input
        placeholder="Describe your idea here"
        class="w-full bg-transparent !outline-none text-[16px] placeholder:tracking-[0.1px] placeholder:text-muted-foreground text-foreground"
        bind:value={projectTitle}
        on:focus={handleFocus}
        on:blur={handleBlur}
      />
      <button
        type="submit"
        class="ml-2 -me-3 hover:invert text-background hover:text-subaccent transition-colors bg-foreground rounded-full p-2 {isProjectTitleValid
          ? ''
          : 'opacity-100 cursor-not-allowed'}"
        aria-label="Send message"
        disabled={!isProjectTitleValid || isLoading}
      >
        <Send size={17} />
      </button>
    </div>

    {#if isFocused}
      <div
        transition:fly={{ y: 10, duration: 500, easing: cubicOut }}
        class="p-2 space-y-5 py-5"
      >
        <div class="relative {isProjectTitleValid ? '' : 'opacity-50'}">
          <User
            size={18}
            class="absolute left-3 top-1/2 -translate-y-1/2 text-[#999]"
          />
          <input
            id="name"
            class="w-full bg-muted rounded-xl !p-3 !pl-10 border border-muted-foreground !outline-none text-foreground pt-6"
            bind:value={name}
            disabled={!isProjectTitleValid}
            on:focus={() => (nameFocused = true)}
            on:blur={() => (nameFocused = false)}
          />
          <label
            for="name"
            class="absolute left-10 text-[#999] transition-all duration-200 {name ||
            nameFocused
              ? 'text-xs hidden -top-2'
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
            class="w-full bg-muted border border-muted-foreground text-foreground rounded-xl !p-3 !pl-10 !outline-none pt-6"
            bind:value={email}
            disabled={!isProjectTitleValid}
            on:focus={() => (emailFocused = true)}
            on:blur={() => (emailFocused = false)}
          />
          <label
            for="email"
            class="absolute left-10 text-[#999] transition-all duration-200 {email ||
            emailFocused
              ? 'text-xs hidden -top-2'
              : 'top-1/2 -translate-y-1/2'}"
          >
            Your Email Address
          </label>
        </div>

        <div class="relative {isProjectTitleValid ? '' : 'opacity-50'}">
          <MessageSquare size={18} class="absolute left-3 top-5 text-[#999]" />
          <textarea
            id="message"
            class="w-full bg-muted border border-muted-foreground rounded-xl !p-0 !pt-4 !pl-10 !outline-none text-foreground resize-none h-24 pt-6"
            bind:value={message}
            disabled={!isProjectTitleValid}
            on:focus={() => (messageFocused = true)}
            on:blur={() => (messageFocused = false)}
          ></textarea>
          <label
            for="message"
            class="absolute left-10 text-[#999] transition-all duration-200 {message ||
            messageFocused
              ? 'text-xs hidden -top-2'
              : 'top-5'}"
          >
            Your Message
          </label>
        </div>

        <div
          class="flex items-center space-x-3 p-3 bg-[#333] rounded-xl {isProjectTitleValid
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
        {#if files && files.length > 0}
          <div class="flex flex-wrap gap-2 px-3">
            {#each Array.from(files) as file, i}
              <div
                class="flex items-center bg-[#444] text-white rounded-full px-3 py-1 text-xs"
              >
                <span class="truncate max-w-[150px]">{file.name}</span>
                <button
                  type="button"
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

<style>
  .fade-up {
    opacity: 0;
    transform: translateX(-50%) translateY(20px);
    animation: fadeUp 0.15s ease-in-out forwards;
  }

  @keyframes fadeUp {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }
</style>
