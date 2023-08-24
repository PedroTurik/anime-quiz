<script lang="ts">
    import { auth, user } from "$lib/firebase";
  
    import { GoogleAuthProvider, signInWithPopup, signOut } from "firebase/auth";
  
    async function signInWithGoogle() {
      const provider = new GoogleAuthProvider();
      const credential = await signInWithPopup(auth, provider);
  
      const idToken = await credential.user.getIdToken();
  
      const res = await fetch("/api/signin", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          // 'CSRF-Token': csrfToken  // HANDLED by sveltekit automatically
        },
        body: JSON.stringify({ idToken }),
      });
    }
  
  </script>


<div class="hero min-h-screen bg-base-200">
  <div class="hero-content text-center">
    <div class="max-w-md">
      <h1 class="text-5xl font-bold">Log in </h1>
      {#if $user}
      <p class="py-6">já está logado</p>
      <a href="/">
              <button class="btn btn-primary" >Volte para home</button>
          </a>
      {:else}
      <p class="py-6">se logue com o google</p>          
          <button class="btn btn-warning" on:click={signInWithGoogle}>Sign In</button>
      {/if}

    </div>
  </div>
</div>