Feature: Logs
    Scenario: Viewing a seeded log file shows its content
        Given a backend log file exists
        And the application is running
        When I open the login page
        And I open the logs page
        And I choose the seeded log file
        Then I should see the log content