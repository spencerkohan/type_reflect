import { z } from 'zod';

export const SDParametersSchema = z.object({
    prompt: z.string(),cfg_scale: z.number(),step_count: z.number(),seed: z.number(),images: z.number(),
});

export type SDParameters = z.infer<typeof SDParametersSchema>;

