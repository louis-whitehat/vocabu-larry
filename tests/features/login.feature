Feature: Login
    Scenario: Selecting a learner and dictionary opens the exam page
        Given learner 'anna' with 'animals' dictionary exists
        And the application is running
        When I open the login page
        And I choose learner 'anna'
        And I choose dictionary 'animals'
        Then I should see the exam page
        And I should see the question for 'dog'

    Scenario: Different learners expose their own dictionaries
        Given learner 'anna' with 'animals' dictionary exists
        And learner 'anna' with dictionary 'colors' containing 'red' as 'rot' exists
        And learner 'zoe' with dictionary 'verbs' containing 'go' as 'gehen' exists
        And the application is running
        When I open the login page
        And I choose learner 'anna'
        Then I should see dictionary 'colors' for the selected learner
        When I choose learner 'zoe'
        And I choose dictionary 'verbs'
        Then I should see the exam page
        And I should see the question for 'go'