Feature: Login
  Scenario: Selecting a learner and dictionary opens the exam page
    Given learner anna with animals dictionary exists
    And the application is running
    When I open the login page
    And I choose learner anna
    And I choose dictionary animals
    Then I should see the exam page
    And I should see the question for dog