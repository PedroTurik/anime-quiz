import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { io } from '$lib';


export const load = (async ( { params } ) => {
    io.emit('join', params.id);

    


    return {};
}) satisfies PageLoad;