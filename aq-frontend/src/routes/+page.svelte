<script lang="ts">
    import { io } from "$lib";
    import { onMount } from "svelte";
    import type { PageData } from "./$types";
    import { goto } from "$app/navigation";

    let roomsID: number;

    function handleJoinRoom() {
        if (roomsID) {
            goto(`/room/${roomsID}`);
        }
    }

    

    export let data: PageData;
</script>

<div class="hero min-h-screen bg-base-200">
    <div class="hero-content text-center">
        <div class="max-w-md">
            <h1 class="text-5xl font-bold">
                <span class="inline-grid">
                    <span
                        class="pointer-events-none col-start-1 row-start-1 bg-[linear-gradient(90deg,theme(colors.error)_0%,theme(colors.secondary)_9%,theme(colors.secondary)_42%,theme(colors.primary)_47%,theme(colors.accent)_100%)] bg-clip-text blur-xl [transform:translate3d(0,0,0)] [-webkit-text-fill-color:transparent] before:content-[attr(data-text)] [@supports(color:oklch(0_0_0))]:bg-[linear-gradient(90deg,oklch(var(--s))_4%,color-mix(in_oklch,oklch(var(--s)),oklch(var(--er)))_22%,oklch(var(--p))_45%,color-mix(in_oklch,oklch(var(--p)),oklch(var(--a)))_67%,oklch(var(--a))_100.2%)]"
                        aria-hidden="true"
                        data-text="Anime Opening Quiz"
                    ></span>
                    <span
                        class="[&amp;::selection]:text-base-content relative col-start-1 row-start-1 bg-[linear-gradient(90deg,theme(colors.error)_0%,theme(colors.secondary)_9%,theme(colors.secondary)_42%,theme(colors.primary)_47%,theme(colors.accent)_100%)] bg-clip-text [-webkit-text-fill-color:transparent] [&amp;::selection]:bg-blue-700/20 [@supports(color:oklch(0_0_0))]:bg-[linear-gradient(90deg,oklch(var(--s))_4%,color-mix(in_oklch,oklch(var(--s)),oklch(var(--er)))_22%,oklch(var(--p))_45%,color-mix(in_oklch,oklch(var(--p)),oklch(var(--a)))_67%,oklch(var(--a))_100.2%)]"
                        >Anime Opening Quiz</span
                    ></span
                >
            </h1>
            <p class="py-6">
                Currently {data.roomsAmount} rooms are active and having fun with
                the quiz.
            </p>
            <form class="flex flex-col justify-center items-center" on:submit|preventDefault={handleJoinRoom}>
                <input bind:value={roomsID} type="number" placeholder="Room Number" class="input input-bordered input-secondary w-full max-w-xs" />
                <br>
                <button class="btn btn-secondary glass btn-wide">Join Room</button>
            </form>
        </div>
    </div>
</div>
