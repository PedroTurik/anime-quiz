import type { PageLoad } from './$types';

export const load = (async ({ params }) => {
    return {
        roomId: params.roomId
    };
}) satisfies PageLoad;