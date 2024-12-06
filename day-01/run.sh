#!/bin/sh

echo "-- Javascript / Typescript"
cd ./js
echo ' > deno task run'
deno task run
echo
cd ..

echo "-- C#"
cd ./c#
echo ' > dotnet run'
dotnet run
cd ..
