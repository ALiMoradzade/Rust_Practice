fn main()
{
    let mut i = 0;

    'frist_loop: loop
    {
        let mut j = 0;

        loop
        {
            if j == 2
            {
                break;
            }
            if i == 2 
            {
                break 'frist_loop;
            }
            j += 1;
        }
        i += 1;
    }
}