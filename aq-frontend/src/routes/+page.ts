import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
    // let roomsAmount = await (await fetch('http://localhost:3000/load_data')).text();
    let roomsAmount = 4
    return { 
        roomsAmount: roomsAmount 
    };
}) satisfies PageLoad;