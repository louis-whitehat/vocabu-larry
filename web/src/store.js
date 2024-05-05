import frenchDictionary from '../../dictionaries/louis/french.txt?raw'
import englishDictionary from '../../dictionaries/louis/english.txt?raw'
import loonyEnglishDictionary from '../../dictionaries/leonie/english.txt?raw'

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
