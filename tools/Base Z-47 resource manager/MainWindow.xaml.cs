using Microsoft.Win32;
using System.IO;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace Base_Z_47_resource_manager
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        private BitmapImage bitmap = null;

        private int w = 320;
        private int h = 200;

        public MainWindow()
        {
            InitializeComponent();
            ToLog("Initialize - OK");
        }

        private void ToLog(string msg)
        {
            LogTextBlock.Text = $"{DateTime.Now}: {msg}\n" + LogTextBlock.Text;
        }

        private void LoadImageBitmap_Click(object sender, RoutedEventArgs e)
        {
            try
            {
                OpenFileDialog dialog = new OpenFileDialog();
                dialog.DefaultDirectory = Environment.CurrentDirectory;

                if (dialog.ShowDialog() == true)
                {
                    Uri uri = new Uri(dialog.FileName);
                    bitmap = new BitmapImage(uri);
                    SrcImage.Source = bitmap;
                    SrcImage.Source = ResizeImage((BitmapSource)SrcImage.Source, 320, 200);
                    ToLog($"Image loaded from: " + dialog.FileName);
                    ToLog($"Image resized to 320x200 pixels");
                    Convert();
                }
            }
            catch (Exception ex)
            {
                ToLog($"Can't load image from selected file: {ex.Message}");
            }
        }

        private BitmapSource ResizeImage(BitmapSource source, int width, int height)
        {
            TransformedBitmap transformedBitmap = new TransformedBitmap();
            transformedBitmap.BeginInit();
            transformedBitmap.Source = source;
            transformedBitmap.Transform = new System.Windows.Media.ScaleTransform(
                (double)width / source.PixelWidth,
                (double)height / source.PixelHeight);
            transformedBitmap.EndInit();
            return transformedBitmap;
        }


        private BitmapImage ConvertToBitmapImage(BitmapSource bitmapSource)
        {
            using (MemoryStream memoryStream = new MemoryStream())
            {
                BitmapEncoder encoder = new BmpBitmapEncoder();
                encoder.Frames.Add(BitmapFrame.Create(bitmapSource));
                encoder.Save(memoryStream);
                memoryStream.Position = 0;

                BitmapImage bitmapImage = new BitmapImage();
                bitmapImage.BeginInit();
                bitmapImage.CacheOption = BitmapCacheOption.OnLoad;
                bitmapImage.StreamSource = memoryStream;
                bitmapImage.EndInit();
                bitmapImage.Freeze();

                return bitmapImage;
            }
        }

        private List<System.Windows.Media.Color> Get24bitVGAPalette()
        {
            List<System.Windows.Media.Color> palette = new List<System.Windows.Media.Color>();

            BitmapImage VGA24PaletteImage = ConvertToBitmapImage((BitmapSource)VGAPalette24bit.Source);
            int wp = (int)(16 * 16) * 4;
            int hp = (int)(16 * 16);
            byte[] VGAPalettePixels = new byte[wp * hp];
            VGA24PaletteImage.CopyPixels(VGAPalettePixels, wp, 0);

            //scan palette
            byte destPixel = 255;

            System.Windows.Media.Color nextColor = System.Windows.Media.Color.FromArgb(0x0, 0, 0, 0);

            for (int py = 0; py < 16 * 16; py += 16)
            {
                for (int px = 0; px < 16 * 16; px += 16)
                {

                    int ofs = (px * 4 + 8) + (py) * 16 * 16 * 4;

                    System.Windows.Media.Color currentColor = System.Windows.Media.Color.FromArgb(
                            VGAPalettePixels[ofs + 3],
                            VGAPalettePixels[ofs + 2],
                            VGAPalettePixels[ofs + 1],
                            VGAPalettePixels[ofs + 0]
                            );
                    if (currentColor != nextColor)
                    {
                        palette.Add(currentColor);
                        nextColor = currentColor;

                        Button panel = new Button();
                        panel.Width = 16;
                        panel.Height = 16;
                        panel.Background = new SolidColorBrush(nextColor);
                        panel.HorizontalAlignment = HorizontalAlignment.Left;
                        panel.VerticalAlignment = VerticalAlignment.Top;
                        panel.Margin = new Thickness(px, py, 0, 0);
                        PalettGrid.Children.Add(panel);
                    }
                }
            }

            return palette;
        }

        private void Convert()
        {
            ToLog("Begin converting");
            try
            {
                VGAPalette24bit.Source = ResizeImage((BitmapSource)VGAPalette24bit.Source, 16 * 16, 16 * 16);

                BitmapPalette VGA24bitPalette = new BitmapPalette(Get24bitVGAPalette());

                ToLog("Get 24bit 256 colors preVGA palette");

                BitmapImage srcBitmapImage = ConvertToBitmapImage(ResizeImage(bitmap, w, h));
                FormatConvertedBitmap srcFormatedBitmapSource = new FormatConvertedBitmap();
                srcFormatedBitmapSource.BeginInit();
                srcFormatedBitmapSource.Source = srcBitmapImage;
                srcFormatedBitmapSource.DestinationPalette = VGA24bitPalette;
                srcFormatedBitmapSource.DestinationFormat = System.Windows.Media.PixelFormats.Indexed8;
                srcFormatedBitmapSource.EndInit();

                SrcImage.Source = srcFormatedBitmapSource;

                ToLog("Image converted to Index8 with 24bit 256 colors palette");

                srcBitmapImage = ConvertToBitmapImage((BitmapSource)SrcImage.Source);

                srcFormatedBitmapSource = new FormatConvertedBitmap();
                srcFormatedBitmapSource.BeginInit();
                srcFormatedBitmapSource.Source = srcBitmapImage;
                srcFormatedBitmapSource.DestinationFormat = System.Windows.Media.PixelFormats.Rgb24;
                srcFormatedBitmapSource.EndInit();

                SrcImage.Source = srcFormatedBitmapSource;

                ToLog("Image converted back to 24bit 256 colors palette");
                ToLog("Palettes linked starting converting to VGA bin image 256 colors native VGA palette");

                BitmapImage srcBitmap = ConvertToBitmapImage((BitmapSource)SrcImage.Source);


                ToLog("Starting join palettes");
                byte[] srcPixels = new byte[w * 4 * h];
                byte[] destPixels = new byte[w * h];
                srcBitmap.CopyPixels(srcPixels, w * 4, 0);

                for (int i = 0; i < srcPixels.Length; i += 4)
                {

                    System.Windows.Media.Color color = System.Windows.Media.Color.FromArgb(
                            srcPixels[i + 3],
                            srcPixels[i + 2],
                            srcPixels[i + 1],
                            srcPixels[i + 0]
                            );

                    destPixels[i / 4] = (byte)VGA24bitPalette.Colors.IndexOf(color);
                }
                ToLog("OK join palettes");
                var destBitmap = BitmapSource.Create(w, h, 96, 96, PixelFormats.Indexed8, VGA24bitPalette, destPixels, w);
                ToLog("OK destination VGA bin 256 colors file");

                DestImage.Source = destBitmap;

                File.WriteAllBytes(@"..\..\..\..\..\logo1.bmp", destPixels);
                ToLog("");
                ToLog(@"Destination file save to: ..\..\..\..\..\logo1.bmp");
            }
            catch (Exception ex)
            {
                ToLog($"Image converting exception: {ex.Message}");
            }
            ToLog("Converting complete. ATENTION! New files is rewrite!");
        }

        private void ConvertButton_Click(object sender, RoutedEventArgs e)
        {
            w = int.Parse(WidthTextBox.Text);
            h = int.Parse(HeightTextBox.Text);
            Convert();
        }
    }
}