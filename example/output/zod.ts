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

export const SimpleEnumsExampleSchema = z.enum([
    SimpleEnumsExample.Foo,
])


export enum StatusCase {
    Initial = "Initial",
    InProgress = "InProgress",
    Complete = "Complete",
    Double = "Double",
    Single = "Single",
}


export const StatusCaseInitialSchema = z.object({
    _case: z.literal(StatusCase.Initial),
});
export type StatusCaseInitial = z.infer<typeof StatusCaseInitialSchema>
            
export const StatusCaseInProgressSchema = z.object({
    _case: z.literal(StatusCase.InProgress),
    data: z.object({
        progress: z.number(),
    shouldConvert: z.bool(),
    })});
export type StatusCaseInProgress = z.infer<typeof StatusCaseInProgressSchema>
            
export const StatusCaseCompleteSchema = z.object({
    _case: z.literal(StatusCase.Complete),
    data: z.object({
        urls: z.array(z.string()),
    })});
export type StatusCaseComplete = z.infer<typeof StatusCaseCompleteSchema>
            
export const StatusCaseDoubleSchema = z.object({
    _case: z.literal(StatusCase.Double),
    data: z.tuple([
            z.number(),
        z.number(),
    ])});
export type StatusCaseDouble = z.infer<typeof StatusCaseDoubleSchema>
            
export const StatusCaseSingleSchema = z.object({
    _case: z.literal(StatusCase.Single),
    data: z.number()});
export type StatusCaseSingle = z.infer<typeof StatusCaseSingleSchema>
            

export const StatusSchema = z.union([
    StatusCaseInitialSchema,
    StatusCaseInProgressSchema,
    StatusCaseCompleteSchema,
    StatusCaseDoubleSchema,
    StatusCaseSingleSchema,
]);
export type Status = z.infer<typeof StatusSchema>
            
