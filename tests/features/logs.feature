Feature: Logs
    Scenario: Login events are written to the logs API
        Given learner 'anna' with 'animals' dictionary exists
        And the application is running
        When I open the login page
        And I choose learner 'anna'
        And I open the logs page
        Then I should see the only available log file selected
        And I should see '1' available log file
        And I should see the log content containing 'LOGIN user=anna'

    Scenario: Request errors are written to the logs API
        Given learner 'anna' with 'animals' dictionary exists
        And the application is running
        When I request missing dictionary 'missing' for learner 'anna'
        And I open the logs page
        Then I should see the only available log file selected
        And I should see '1' available log file
        And I should see the log content containing '/api/dictionary'
        And I should see the log content containing 'dictionary does not exist'