<script lang="ts">
    import { db, roomData, user } from "$lib/firebase";
    import { onDisconnect, ref, set } from "firebase/database";

    async function writeUserData(userId: any, name: any, imageUrl: any) {
        await set(ref(db, "mainroom/users/" + userId), {
            username: name,
            profile_picture: imageUrl,
            points: 0,
        });
    }

    onDisconnect(ref(db, "mainroom/users/" + $user?.uid))
        .remove()
        .catch((e) => console.log(e));

    let connected_users: [ string,{
            username: string;
            profile_picture: string;
            points: number;
        }][] = [];

    $: if ($roomData?.users) {
        connected_users = Object.entries($roomData?.users);
    }
</script>

{#await writeUserData($user?.uid, $user?.displayName, $user?.photoURL)}
    <span
        class="loading loading-spinner loading-lg absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
    />
{:then _}
    <div class="flex flex-col text-center">
        {#each connected_users as [k, v]}
            <div>
                <p>{k}</p>
                <p>{v.username}</p>
            </div>
        {/each}
    </div>
{/await}
