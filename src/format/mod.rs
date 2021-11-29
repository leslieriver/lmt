pub fn return_ending(input: String) -> String{
    let mut ret_val : String = "".to_string();

    if input.contains("jpg")  {
        let last_three: String = input.chars().rev().take(3).collect();

        if last_three == "gpj".to_string(){
            ret_val = "jpg".to_string();
        }
    }

    if input.contains("gif")  {
        let last_three: String = input.chars().rev().take(3).collect();

        if last_three == "fig".to_string(){
            ret_val = "gif".to_string();
        }
    }

    // Handle case where image has a question mark at end for some odd reason
    if input.contains("jpg?")  {
        let last_three: String = input.chars().rev().take(4).collect();

        if last_three == "?gpj".to_string(){
            ret_val = "jpg".to_string();
        }
    }

    if input.contains("png?")  {
        let last_four: String = input.chars().rev().take(4).collect();

        if last_four == "?gnp".to_string(){
            ret_val = "png".to_string();
        }
    }

    if input.contains("png")  {
        let last_three: String = input.chars().rev().take(3).collect();

        if last_three == "gnp".to_string(){
            ret_val = "png".to_string();
        }
    }
    
    if input.contains("webp")  {
        let last_four: String = input.chars().rev().take(4).collect();

        if last_four == "pbew".to_string(){
            ret_val = "webp".to_string();
        }
    }

    if input.contains("webp?")  {
        let last_five: String = input.chars().rev().take(5).collect();

        if last_five == "?pbew".to_string(){
            ret_val = "webp".to_string();
        }
    }

    if input.contains("mp4")  {
        let last_three: String = input.chars().rev().take(3).collect();

        if last_three == "4pm".to_string(){
            ret_val = "mp4".to_string();
        }
    }

    if input.contains("mp4?")  {
        let last_four: String = input.chars().rev().take(4).collect();

        if last_four == "?4pm".to_string(){
            ret_val = "mp4".to_string();
        }
    }

    if ret_val == "".to_string() {
        println!("Missed extension for url {}",input);
    }
    return ret_val;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jpg() {
        assert_eq!(return_ending("image.jpg".to_string()), "jpg".to_string());
    }

    #[test]
    fn test_gif() {
        assert_eq!(return_ending("image.gif".to_string()), "gif".to_string());
    }

    #[test]
    fn test_jpg_question_mark() {
        assert_eq!(return_ending("image.jpg?".to_string()), "jpg".to_string());
    }

    #[test]
    fn test_png() {
        assert_eq!(return_ending("image.png".to_string()), "png".to_string());
    }

    #[test]
    fn test_webp() {
        assert_eq!(return_ending("image.webp".to_string()), "webp".to_string());
    }


    #[test]
    fn test_multi() {
        assert_eq!(return_ending("image.webp.jpg.png".to_string()), "png".to_string());
    }
}