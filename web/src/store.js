import frenchDictionary from '../../dictionaries/french.txt?raw'
import englishDictionary from '../../dictionaries/english.txt?raw'
import loonyEnglishDictionary from '../../dictionaries/loony-english.txt?raw'

const store =  {
  dictionaries: [
    {
      name: 'french',
      content: frenchDictionary
    },
    {
      name: 'english',
      content: englishDictionary
    },
    {
      name: 'loony english',
      content: loonyEnglishDictionary
    }
  ]
}

export default store
