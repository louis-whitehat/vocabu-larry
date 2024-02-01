
open System
open System.IO

let database = fsi.CommandLineArgs[1]

let readPair (line:string) =
    let elements = line.Split([| ':' |])
    if elements.Length <> 2 then elements |> failwithf "Exactly 2 elements expected but got '%A'"
    elements.[0].Trim(), elements.[1].Trim()

let vocables =
    File.ReadAllLines(database)
    |> Seq.map readPair
    |> List.ofSeq

let rand = new Random(DateTime.UtcNow.Ticks |> int)

while true do
    let idx = rand.Next(vocables |> List.length) 
    let vocable = vocables.[idx] 

    let question, answer =
        if rand.Next(2) = 0 then    
            vocable |> fst, vocable |> snd
        else
            vocable |> snd, vocable |> fst

    question |> printf "%s ? "

    if answer = Console.ReadLine() then
        printfn "CORRECT ;-)"
    else
        answer |> printfn "WRONG :(    correct answer is '%s'" 
