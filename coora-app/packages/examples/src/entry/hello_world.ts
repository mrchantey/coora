//@ts-ignore external
@external("host", "howdy")
export declare function howdy(val: i32): void

export function hello(): void{
	const a = 2
	const b = 10
	const c = add(a, b)
	howdy(a + b)
}

export function add(a: u64, b: u64): u64{
	return a + b
}