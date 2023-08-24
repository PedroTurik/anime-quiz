<script>
    import AuthCheck from "$lib/components/AuthCheck.svelte";
    import { auth, user } from "$lib/firebase";
    import { signOut } from "firebase/auth";
    async function signOutSSR() {
        const res = await fetch("/api/signin", { method: "DELETE" });
        await signOut(auth);
    }
  
</script>
{#if $user}
    <button on:click={signOutSSR} class="btn btn-accent absolute">
        Log Out</button
    >
{/if}
<div class="hero min-h-screen bg-base-200">
    <div class="hero-content text-center">
      <div class="max-w-md">
        <h1 class="text-5xl font-bold">Anime Quiz</h1>
        <p class="py-6">acerte as openings para vencer e bla bla bla chupem meu tiquinho durinho poggerzinho</p>
        <AuthCheck>
          <a href="/mainroom">
            <button class="btn btn-primary">Começar</button>
          </a>
        </AuthCheck>
      </div>
    </div>
  </div>

