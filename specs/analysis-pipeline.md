```mermaid
graph TB
subgraph ab7778a2d708eb0f["Analyse"]
207a7feca601c2e2["Which MatcherRules to use and use keywords > Ask GPT."]
78e957bda4884a41["Get groups of keywords []."]
f755572270f4b40b["Which groups are related to the question? > Ask GPT."]
223a839673882ebc["Get all keywords from groups []."]
876ae3132c378cae["Which keywords are related to the question? > Ask GPT."]
c5eacc9b622b4c0a["Get notes by keywords and get summary. If not present generate it > Ask GPT."]
76f53107dccbc61b["Get notes description > Ask GPT and put it in the memory."]

end
2e78d5081acb9130["> ANALYSE"]
03c0bf2d7d53ba30["Ask GPT what type of replay?"]
5e4aff86d5515375["USER MESSAGE"]
b6f82d11aa769a1f["Can answer using memory?"]
0ae8cee0a7cf4f85["Short Memory"]
207a7feca601c2e2["Which MatcherRules to use and use keywords > Ask GPT."]
78e957bda4884a41["Get groups of keywords []."]
f755572270f4b40b["Which groups are related to the question? > Ask GPT."]
223a839673882ebc["Get all keywords from groups []."]
876ae3132c378cae["Which keywords are related to the question? > Ask GPT."]
c5eacc9b622b4c0a["Get notes by keywords and get summary. If not present generate it > Ask GPT."]
76f53107dccbc61b["Get notes description > Ask GPT and put it in the memory."]
e5a6cbc4df30ca71[">DO SPECIAL"]
4f6683862d28ae89["> STANDARD REPLAY"]
e0655c30dc3992a5["Use chat context"]
06a04622a782b489["do some special stuff: add new note, delete note, open note"]
2e78d5081acb9130 --> b6f82d11aa769a1f
f755572270f4b40b --> 223a839673882ebc
223a839673882ebc --> 876ae3132c378cae
876ae3132c378cae --> 207a7feca601c2e2
207a7feca601c2e2 --> c5eacc9b622b4c0a
c5eacc9b622b4c0a --> 76f53107dccbc61b
e5a6cbc4df30ca71 --> 06a04622a782b489
b6f82d11aa769a1f --> |Yes| 4f6683862d28ae89
78e957bda4884a41 --> f755572270f4b40b
b6f82d11aa769a1f --> |No
| ab7778a2d708eb0f
4f6683862d28ae89 --> e0655c30dc3992a5
5e4aff86d5515375 --> 03c0bf2d7d53ba30
03c0bf2d7d53ba30 --> 2e78d5081acb9130
03c0bf2d7d53ba30 --> 4f6683862d28ae89
03c0bf2d7d53ba30 --> e5a6cbc4df30ca71
76f53107dccbc61b --> 0ae8cee0a7cf4f85
0ae8cee0a7cf4f85 <--> b6f82d11aa769a1f
style ab7778a2d708eb0f fill:#2b7e7d, stroke:#004b4a
style 207a7feca601c2e2 fill:#543e87, stroke:#210b54
style 78e957bda4884a41 fill:#83812c, stroke:#504e00
style f755572270f4b40b fill:#543e87, stroke:#210b54
style 223a839673882ebc fill:#83812c, stroke:#504e00
style 876ae3132c378cae fill:#543e87, stroke:#210b54
style c5eacc9b622b4c0a fill:#543e87, stroke:#210b54
style 76f53107dccbc61b fill:#543e87, stroke:#210b54
style 2e78d5081acb9130 fill:#2b7e7d, stroke:#004b4a
style 03c0bf2d7d53ba30 fill:#543e87, stroke:#210b54
style 5e4aff86d5515375 fill:#9d6b35, stroke:#6a3802
style b6f82d11aa769a1f fill:#543e87, stroke:#210b54
style 207a7feca601c2e2 fill:#543e87, stroke:#210b54
style 78e957bda4884a41 fill:#83812c, stroke:#504e00
style f755572270f4b40b fill:#543e87, stroke:#210b54
style 223a839673882ebc fill:#83812c, stroke:#504e00
style 876ae3132c378cae fill:#543e87, stroke:#210b54
style c5eacc9b622b4c0a fill:#543e87, stroke:#210b54
style 76f53107dccbc61b fill:#543e87, stroke:#210b54
style e5a6cbc4df30ca71 fill:#207038, stroke:#003d05
style 4f6683862d28ae89 fill:#784445, stroke:#451112
style e0655c30dc3992a5 fill:#784445, stroke:#451112
style 06a04622a782b489 fill:#207038, stroke:#003d05
```