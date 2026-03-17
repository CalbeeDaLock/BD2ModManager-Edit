
export function timeOperation<T>(operation: () => T, operationName: string, log: boolean = true): T {
    const start = performance.now();
    const result = operation();
    const end = performance.now();
    if (log) {
        console.log(`[TIMING] ${operationName}: ${(end - start).toFixed(2)}ms`);
    }
    return result;
}

export async function timeAsyncOperation<T>(operation: () => Promise<T>, operationName: string): Promise<T> {
    const start = performance.now();
    const result = await operation();
    const end = performance.now();
    console.log(`[TIMING] ${operationName}: ${(end - start).toFixed(2)}ms`);
    return result;
}
