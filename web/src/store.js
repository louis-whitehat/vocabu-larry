import frenchDictionary from '../../dictionaries/french.txt?raw'
import englishDictionary from '../../dictionaries/english.txt?raw'
import loonyEnglishDictionary from '../../dictionaries/loony-english.txt?raw'

const store = {
  users: [
    {
      name: 'Louis',
      dictionaries: [
        {
          name: 'French',
          content: frenchDictionary
        },
        {
          name: 'English',
          content: englishDictionary
        }
      ]
    },
    {
      name: 'Leonie',
      dictionaries: [
        {
          name: 'English',
          content: loonyEnglishDictionary
        }
      ]
    }
  ]
}

export default store
