# capybo/usprotocol

A lightweight and versatile protocol for client-server interaction. 
> This repository provides a reliable foundation for performing basic operations and exchanging messages between nodes in distributed systems.
> Its ease of use and flexibility make USP an excellent choice for developers aiming for fast and efficient data exchange in their applications.

## Disclaimer
This protocol has been developed solely for educational purposes.

> [!CAUTION]
> It is not intended or recommended for real-world, production, or mission-critical applications.
> The protocol lacks the robustness, security features, and optimizations required for deployment in professional settings.

### Important Points:
1. **Educational Use Only**: The USP protocol is designed to aid learning and understanding of basic client-server communication concepts. It is not suitable for operational use in live systems.
2. **Security Considerations**: The protocol does not implement security measures such as encryption, authentication, or authorization. Using it in a production environment could expose sensitive data to potential risks.
3. **Performance Concerns**: The protocol prioritizes simplicity over performance optimizations. As such, it may not be suitable for applications requiring high efficiency or low-latency communication.
4. **Modification and Extension**: Feel free to modify and extend the protocol for educational purposes. However, exercise caution if considering its application in real-world scenarios.

## Commands
**Compare Words (COMPARE_WORDS):**
   - Command: `COMPARE_WORDS word1 word2`
   - Parameters:
      - `word1`: The first word.
      - `word2`: The second word.
   - Description: Сompares these words and returns "true" if they are identical in terms of both the number of letters and their order, and "false" otherwise.

## Response Codes
1. **Success (OK):**
   - Code: `100`
   - Description: The request was successfully processed.

2. **Error (ERROR):**
   - Code: `200`
   - Description: General error in the request or processing.

3. **Unknown Command (UNKNOWN COMMAND):**
   - Code: `201`
   - Description: The command specified in the request is not recognized.

4. **Invalid Parameters (INVALID PARAMETERS):**
   - Code: `202`
   - Description: One or more parameters in the request have an incorrect format or value.

5. **Server Busy (SERVER BUSY):**
   - Code: `300`
   - Description: The server is temporarily busy and cannot process the request at the moment.
