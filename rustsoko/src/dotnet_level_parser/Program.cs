//Sokoban resource level parser
//Original levels from https://github.com/begoon/sokoban-maps/blob/master/maps/sokoban-maps-60-plain.txt

using System;

namespace LevelParser
{
    internal class Program
    {
        private static string fileName = "sokoban-maps-60-plain.txt";

        private static string storeLevelfileName = @"..\..\..\..\levels\level{0}.rs";

        

        private static string levelDelimiter = "*************************************";
        private static string levelMazeMask = "Maze";
        private static string levelLengthMask = "Length";


        static void Main(string[] args)
        {
            Console.WriteLine("RustSoko levels parser");

            if (File.Exists(fileName))
            {
                List<string> lines = File.ReadLines(fileName).ToList();

                if (lines.Any())
                {
                    int levelsCount = 0;
                    List<string> levelLines = new List<string>();
                    List<string> levelFile = new List<string>();
                    List<string> modFile = new List<string>();
                    List<string> insertFile = new List<string>();
                    for (int i = 0; i < lines.Count; i++)
                    {
                        string line = lines[i]; 

                        if (line.StartsWith(levelDelimiter))
                        {
                            if (levelLines.Any())
                            {
                                levelFile.Clear();
                                levelFile.Add("pub fn get() -> [[char; 30]; 20] {");
                                levelFile.Add("return");
                                levelFile.Add("[");                                
                                foreach(string row in levelLines)
                                {
                                    string rs_row = "\t[";
                                    foreach(char col in row)
                                    {
                                        char decodeCol = ' ';
                                        if (col == '@')
                                        {
                                            decodeCol = '@';
                                        }
                                        else
                                        if (col == 'X')
                                        {
                                            decodeCol = '#';
                                        }
                                        else
                                        if (col == '*')
                                        {
                                            decodeCol = '$';
                                        }
                                        else
                                        if (col == '.')
                                        {
                                            decodeCol = '.';
                                        }

                                        rs_row += $"'{decodeCol}',";
                                    }
                                    for (int j = 0; j < 30 - row.Length; j++)
                                    {
                                        rs_row += $"' ',";
                                    }
                                    rs_row = rs_row.Remove(rs_row.Length - 1);
                                    rs_row += "],";
                                    levelFile.Add(rs_row);
                                }

                                for (int r = 0; r < 20 - levelLines.Count; r++)
                                {
                                    string rs_row = "\t[";
                                    for (int j = 0; j < 30; j++)
                                    {
                                        rs_row += $"' ',";
                                    }
                                    rs_row = rs_row.Remove(rs_row.Length - 1);
                                    rs_row += "],";
                                    levelFile.Add(rs_row);
                                }

                                levelFile.Add("];");
                                levelFile.Add("}");
                                File.WriteAllLines(string.Format(storeLevelfileName, levelsCount), levelFile);
                                modFile.Add($"mod level{levelsCount};");
                                insertFile.Add($"\tlevels.insert(\"level{levelsCount}\".to_string(), level{levelsCount}::get());");
                            }
                            levelsCount++;
                            levelLines.Clear();
                        }

                        if (!string.IsNullOrEmpty(line) && levelLines.Any())
                        {
                            levelLines.Add(line);
                        }

                        if (line.Contains(levelLengthMask))
                        {
                            i+=2;
                            line = lines[i];
                            levelLines.Add(line);
                        }
                    }
                    modFile.AddRange(insertFile);
                    File.WriteAllLines(string.Format(storeLevelfileName + "_mod" , levelsCount), modFile);
                }
                else
                {
                    Console.WriteLine("Fail: file {0} is empty.", fileName);
                }
            }
            else
            {
                Console.WriteLine("Fail: file {0} not exists in work folder.", fileName);
            }
        }
    }
}