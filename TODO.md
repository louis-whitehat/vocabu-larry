

- do the dictionary parsing in backend
  - send parsed json to FE
  - add more tolerance while parsing
    - trim whitespaces at start and end
    - remove duplicated whitespaces in between words
  - add tests for parsing - at least one happy path

- do the exam trough backend
  - ask for new word, handle score

- do not fully take words randomly
  put a bit more weight on failures

- give more hints than just number of words, like
  - first letter

- compare verbs in english without "to" 

- automatically suggest other tenses


