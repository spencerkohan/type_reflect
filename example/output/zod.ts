import { z } from 'zod';

export const SDParametersSchema = z.object({
  prompt: z.string(),
  negative_prompt: z.string().optional(),
  cfg_scale: z.number(),
  step_count: z.number(),
  seed: z.number(),
  images: z.number(),
  results: z.array(z.string()),
  headers: z.map(z.string(), z.string()),
});

export type SDParameters = z.infer<typeof SDParametersSchema>;


export enum SimpleEnumsExample {
  Foo = "Foo",
}

export const SimpleEnumsExampleScema = z.enum([
  SimpleEnumsExample.Foo,
])


export enum StatusCase {
  Initial = "Initial",
  InProgress = "InProgress",
  Complete = "Complete",
}


export const InitialScema = z.object({
  _case: StatusCase.Initial,
});
export type Initial = z.infer<typeof InitialScema>

export const InProgressScema = z.object({
  _case: StatusCase.InProgress,
  data: z.object({
    progress: z.number(),
  })
});
export type InProgress = z.infer<typeof InProgressScema>

export const CompleteScema = z.object({
  _case: StatusCase.Complete,
  data: z.object({
    urls: z.array(z.string()),
  })
});
export type Complete = z.infer<typeof CompleteScema>


export const StatusScema = z.union([
  InitialScema,
  InProgressScema,
  CompleteScema,
]);
export type Status = z.infer<typeof StatusScema>
