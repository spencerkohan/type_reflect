import { z } from 'zod';

export const SDParametersSchema = z.object({
    prompt: z.string(),
    negativePrompt: z.string().optional(),
    cfgScale: z.number(),
    stepCount: z.number(),
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
    Double = "Double",
    Single = "Single",
}


export const InitialScema = z.object({
    _case: z.literal(StatusCase.Initial),
});
export type Initial = z.infer<typeof InitialScema>
            
export const InProgressScema = z.object({
    _case: z.literal(StatusCase.InProgress),
    data: z.object({
    progress: z.number(),
    })});
export type InProgress = z.infer<typeof InProgressScema>
            
export const CompleteScema = z.object({
    _case: z.literal(StatusCase.Complete),
    data: z.object({
    urls: z.array(z.string()),
    })});
export type Complete = z.infer<typeof CompleteScema>
            
export const DoubleScema = z.object({
    _case: z.literal(StatusCase.Double),
    data: z.tuple([
        z.number(),
        z.number(),
    ])});
export type Double = z.infer<typeof DoubleScema>
            
export const SingleScema = z.object({
    _case: z.literal(StatusCase.Single),
    data: z.number()});
export type Single = z.infer<typeof SingleScema>
            

export const StatusScema = z.union([
    InitialScema,
    InProgressScema,
    CompleteScema,
    DoubleScema,
    SingleScema,
]);
export type Status = z.infer<typeof StatusScema>
            
