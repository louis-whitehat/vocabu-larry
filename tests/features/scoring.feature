Feature: Scoring
    Scenario: Correct answer is reflected on the score page
        Given learner 'anna' with 'animals' dictionary exists
        And the application is running
        When I open the login page
        And I choose learner 'anna'
        And I choose dictionary 'animals'
        And I answer 'Hund'
        Then I should see that the answer was correct
        When I finish the exam
        Then I should see the score page
        And the score table should show 'animals' with 1 correct out of 1